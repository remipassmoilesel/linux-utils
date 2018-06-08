function conky_get_cpu_graph_template()
    return {
        kind = 'ring_graph',
        conky_value = 'cpu cpu1',
        critical_threshold = 60,
        --
        center = { x = 140, y = 455 },
        radius = 20,
        --
        background_color = 0xFFFFFF,
        background_alpha = 0.7,
        background_thickness = 2,
        --
        bar_color = 0xFFFFFF,
        bar_alpha = 1,
        bar_thickness = 8,
        --
        start_angle = 210,
        end_angle = 360,
        --
        bar_color_critical = 0xFFA0A0,
        bar_alpha_critical = 1,
        bar_thickness_critical = 8,
        --
        background_color_critical = 0xFFA0A0,
        background_alpha_critical = 0.7,
        background_thickness_critical = 2,
    }
end

function conky_get_cpu_graph_configurations()
    local graphGap = 15
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



local cpuGraphs = conky_get_cpu_graph_configurations()

elements = cpuGraphs