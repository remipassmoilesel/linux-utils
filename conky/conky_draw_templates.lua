
template_config = {
    critical_color = 0xFFA0A0,
    normal_color = 0xFFFFFF
}

function conky_get_cpu_graph_template()
    local largeBarThickness = 10
    local thinBarThickness = 2
    local fgAlpha = 0.9
    local bgAlpha = 0.3
    return {
        kind = 'ring_graph',
        critical_threshold = 70,
        change_color_on_critical = true,
        --
        center = { x = 155, y = 530 },
        radius = 20,
        --
        start_angle = 210,
        end_angle = 360,
        --
        bar_color = template_config.normal_color,
        bar_alpha = fgAlpha,
        bar_thickness = largeBarThickness,
        --
        background_color = template_config.normal_color,
        background_alpha = bgAlpha,
        background_thickness = thinBarThickness,
        --
        bar_color_critical = template_config.critical_color,
        bar_alpha_critical = fgAlpha,
        bar_thickness_critical = largeBarThickness,
        --
        background_color_critical = template_config.critical_color,
        background_alpha_critical = bgAlpha,
        background_thickness_critical = thinBarThickness,
    }
end

function conky_get_bar_template()
    return {
        kind = 'bar_graph',
        change_color_on_critical = true,
        --
        background_thickness = 8,
        background_color = template_config.normal_color,
        background_alpha = 0.3,
        --
        bar_color = template_config.normal_color,
        bar_alpha = 0.9,
        bar_thickness = 8,
        --
        bar_color_critical = template_config.critical_color,
        bar_alpha_critical = 0.9,
        bar_thickness_critical = 8,
        --
        background_color_critical = template_config.critical_color,
        background_alpha_critical = 0.3,
        background_thickness_critical = 8,
    }
end