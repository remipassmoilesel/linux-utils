function conky_get_cpu_graph_template()
    return {
        kind = 'ring_graph',
        conky_value = 'cpu cpu1',
        critical_threshold = 70,
        change_color_on_critical = true,
        --
        center = { x = 155, y = 530 },
        radius = 20,
        --
        background_color = 0xFFFFFF,
        background_alpha = 0.4,
        background_thickness = 2,
        --
        bar_color = 0xFFFFFF,
        bar_alpha = 0.9,
        bar_thickness = 8,
        --
        start_angle = 210,
        end_angle = 360,
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

function conky_get_cpu_graph_configurations()

    local graphGap = 13

    local cpuGraphsConfigurations = {}
    local cpuNumber = tonumber(conky_command('grep processor /proc/cpuinfo | wc -l '))

    for cpuIndex = 0, cpuNumber do
        local config = conky_get_cpu_graph_template()
        config.conky_value = 'cpu cpu' .. cpuIndex
        config.radius = config.radius + cpuIndex * graphGap
        table.insert(cpuGraphsConfigurations, config)
    end

    return cpuGraphsConfigurations
end

function conky_get_temperature_graph()
    return {
        kind = 'bar_graph',
        conky_value = 'acpitemp',
        critical_threshold = 70,
        max_value = 90,
        change_color_on_critical = true,
        --
        from = {x = 220, y = 915},
        to = {x = 320, y = 915},
        --
        background_thickness = 8,
        background_color = 0xFFFFFF,
        background_alpha = 0.4,
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
        background_alpha_critical = 0.4,
        background_thickness_critical = 8,
    }
end

local allGraphs = conky_get_cpu_graph_configurations()
local tempGraph = conky_get_temperature_graph()

table.insert(allGraphs, tempGraph)

elements = allGraphs
