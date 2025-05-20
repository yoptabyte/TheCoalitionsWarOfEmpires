use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::components::{PlacementPending, PendingPurchase};
use crate::game::{Ground, ClickCircle, ShapeType};
use crate::input::MouseWorldPosition;
use crate::ui::money_ui::place_shape;

/// Система для обновления позиции объектов ожидающих размещения
pub fn update_pending_placement_position(
    mouse_world_position: Res<MouseWorldPosition>,
    mut query: Query<(&mut Transform, &ShapeType), With<PlacementPending>>,
) {
    // Если нет позиции мыши, выходим
    if mouse_world_position.0.is_none() {
        return;
    }
    
    let mouse_pos = mouse_world_position.0.unwrap();
    
    for (mut transform, shape_type) in query.iter_mut() {
        // Рассчитываем базовую высоту в зависимости от типа объекта
        let base_height = match shape_type {
            ShapeType::Cube => 0.5,
            ShapeType::Infantry => 0.5,
            ShapeType::Airplane => 10.0,
            ShapeType::Tower => 1.5,
            ShapeType::Farm => 0.5,
            ShapeType::Mine => 1.5,
            ShapeType::SteelFactory => 2.0,
            ShapeType::PetrochemicalPlant => 1.75,
            ShapeType::Trench => 0.25,
        };
        
        // Устанавливаем позицию объекта под курсором
        transform.translation = Vec3::new(
            mouse_pos.x,
            base_height,
            mouse_pos.z
        );
    }
}

/// Система для обработки клика по земле при размещении нового объекта
/// ВАЖНО: Эта система обрабатывает ТОЛЬКО клики для размещения новых объектов,
/// а не для перемещения существующих (которые обрабатываются в handle_ground_clicks)
pub fn handle_placement_clicks(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Click>>,
    query_ground: Query<(), With<Ground>>,
    pending_placement_query: Query<(Entity, &ShapeType), With<PlacementPending>>,
    mut pending_purchase: ResMut<PendingPurchase>,
    mut click_circle: ResMut<ClickCircle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    // Обработаем сначала случай, когда есть ожидающая покупка (PendingPurchase)
    if pending_purchase.shape_type.is_some() {
        let mut clicked_on_ground = false;
        let mut ground_click_position: Option<Vec3> = None;
        
        for event in click_events.read() {
            // Обрабатываем только левые клики
            if event.button != PointerButton::Primary {
                continue;
            }
            
            info!("handle_placement_clicks: Processing placement click event on target {:?}", event.target);
            
            if query_ground.get(event.target).is_ok() {
                clicked_on_ground = true;
                if let Some(position) = event.hit.position {
                    ground_click_position = Some(position);
                    info!("handle_placement_clicks: Ground selected for object placement at position {:?}", position);
                } else {
                    info!("handle_placement_clicks: Clicked on ground for placement but no position information available");
                }
            }
        }
        
        if clicked_on_ground && ground_click_position.is_some() {
            let target_position = ground_click_position.unwrap();
            
            // Используем as_ref() для получения ссылки, а не владения
            if let Some(shape_type) = pending_purchase.shape_type.as_ref() {
                info!("handle_placement_clicks: Placing new object of type {:?} at position {:?}", shape_type, target_position);
                
                // Создаем объект на позиции клика, используя упрощенную функцию
                place_shape(
                    &mut commands, 
                    shape_type.clone(), 
                    target_position, 
                    &mut meshes, 
                    &mut materials
                );
                
                // Обновляем информацию для отображения круга клика
                click_circle.position = Some(target_position);
                click_circle.spawn_time = Some(time.elapsed_seconds());
                
                // Сбрасываем ожидающую покупку
                pending_purchase.shape_type = None;
                pending_purchase.cost_paid = false;
                
                info!("handle_placement_clicks: Object placed successfully, placement mode deactivated");
                return; // Выходим, так как покупка уже обработана
            }
        }
    }
    
    // Если нет объектов с компонентом PlacementPending, выходим
    // Это ветка для обратной совместимости
    if pending_placement_query.is_empty() {
        return;
    }

    info!("handle_placement_clicks: Processing click events for {} older-style pending placement objects", pending_placement_query.iter().count());

    let mut clicked_on_ground = false;
    let mut ground_click_position: Option<Vec3> = None;
    
    for event in click_events.read() {
        // Обрабатываем только левые клики
        if event.button != PointerButton::Primary {
            continue;
        }
        
        info!("handle_placement_clicks: Processing click event on target {:?}", event.target);
        
        if query_ground.get(event.target).is_ok() {
            clicked_on_ground = true;
            if let Some(position) = event.hit.position {
                ground_click_position = Some(position);
                info!("handle_placement_clicks: Clicked on ground at position {:?}", position);
            } else {
                info!("handle_placement_clicks: Clicked on ground but no position information available");
            }
        }
    }
    
    if clicked_on_ground && ground_click_position.is_some() {
        let target_position = ground_click_position.unwrap();
        
        // Получаем первый объект, ожидающий размещения
        if let Some((entity, shape_type)) = pending_placement_query.iter().next() {
            info!("handle_placement_clicks: Placing object of type {:?} at position {:?}", shape_type, target_position);
            
            // Удаляем компонент ожидания размещения и обновляем позицию
            commands.entity(entity)
                .remove::<PlacementPending>();
            
            // Устанавливаем позицию объекта
            commands.entity(entity)
                .insert(Transform::from_translation(target_position));
            
            // Обновляем информацию для отображения круга клика
            click_circle.position = Some(target_position);
            click_circle.spawn_time = Some(time.elapsed_seconds());
        }
    } else if !clicked_on_ground {
        info!("handle_placement_clicks: No click on ground detected");
    }
} 