function conky_get_cpu_graph_template()
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
        bar_color = 0xFFFFFF,
        bar_alpha = 0.9,
        bar_thickness = 8,
        --
        background_color = 0xFFFFFF,
        background_alpha = 0.4,
        background_thickness = 2,
        --
        bar_color_critical = 0xFFA0A0,
        bar_alpha_critical = 0.9,
        bar_thickness_critical = 8,
        --
        background_color_critical = 0xFFA0A0,
        background_alpha_critical = 0.4,
        background_thickness_critical = 2,
    }
end

function conky_get_bar_template()
    return {
        kind = 'bar_graph',
        change_color_on_critical = true,
        --
        background_thickness = 8,
        background_color = 0xFFFFFF,
        background_alpha = 0.3,
        --
        bar_color = 0xFFFFFF,
        bar_alpha = 0.9,
        bar_thickness = 8,
        --
        bar_color_critical = 0xFFA0A0,
        bar_alpha_critical = 0.9,
        bar_thickness_critical = 8,
        --
        background_color_critical = 0xFFA0A0,
        background_alpha_critical = 0.3,
        background_thickness_critical = 8,
    }
end