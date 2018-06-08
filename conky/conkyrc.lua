conky.config = {
    use_xft = true,
    xftalpha = .1,
    update_interval = 0.6,
    total_run_times = 0,

    own_window = true,
    own_window_type = 'panel',
    own_window_transparent = true,
    own_window_hints = 'undecorated,below,sticky,skip_taskbar,skip_pager',
    own_window_colour = '000000',
    own_window_argb_visual = true,
    own_window_argb_value = 0,

    double_buffer = true,

    minimum_width = 320,
    maximum_width = 320,

    minimum_height = 10,

    draw_shades = false,
    draw_outline = false,
    draw_borders = false,
    draw_graph_borders = true,

    default_color = 'CCCCCC',
    default_shade_color = '333333',
    default_outline_color = 'black',
    color1 = 'FFFFFF',

    alignment = 'top_right',
    gap_x = 25,
    gap_y = 0,
    no_buffers = true,
    text_buffer_size = 2048,
    uppercase = false,
    cpu_avg_samples = 4,
    net_avg_samples = 2,
    override_utf8_locale = true,

    font = 'Ubuntu:style=thin:size=10',

    lua_load = '~/.conky/remipassmoilesel/conky_main.lua',
    lua_draw_hook_pre = 'main',
}

conky.text = [[

${color1}
${voffset 10}
${alignr}${font Ubuntu:style=Medium:pixelsize=38}${time %H:%M}${font}
${voffset 2}
${alignr}${font Ubuntu:style=Medium:pixelsize=13}${time %A %d %B %Y}${font}
${voffset 2}
${font FontAwesome}${font}  ${font Ubuntu:style=Medium:pixelsize=15}NETWORK${font}
${hr}

${voffset 4}PUBLIC IP ${alignr}${lua_parse conky_public_address}
${voffset 4}PRIVATE IPs ${alignr}${lua_parse conky_private_addresses}

${font FontAwesome}${font} ${downspeed wlp4s0} ${alignr} ${upspeed wlp4s0} ${font FontAwesome}${font}
${downspeedgraph wlp4s0 40,160 CCCCCC 0099FF -t}${color} ${upspeedgraph wlp4s0 40,160 CCCCCC 0099FF -t}${color}

${font FontAwesome}${font}  ${font Ubuntu:style=Medium:pixelsize=15}CPU${font}
${hr}
${voffset 120}${offset 50}${font Ubuntu:style=Medium:pixelsize=18}${cpu}%${font}

${voffset 4}TOP USAGE
${voffset 4}1: ${top name 1}${alignr} ${top pid 1} ${top mem 1}
${voffset 4}2: ${top name 2}${alignr} ${top pid 2} ${top mem 2}
${voffset 4}3: ${top name 3}${alignr} ${top pid 3} ${top mem 3}


${font FontAwesome}${font}  ${font Ubuntu:style=Medium:pixelsize=15}MEMORY${font}
${hr}

${voffset 4}${memperc}%/${memmax} ${alignr}${membar 14,160}

${voffset 4}TOP USAGE
${voffset 4}1: ${top_mem name 1}${alignr} ${top_mem pid 1} ${top_mem mem 1}
${voffset 4}2: ${top_mem name 2}${alignr} ${top_mem pid 2} ${top_mem mem 2}
${voffset 4}3: ${top_mem name 3}${alignr} ${top_mem pid 3} ${top_mem mem 3}


${font FontAwesome}${font}  ${font Ubuntu:style=Medium:pixelsize=15}SYSTEM${font}
${hr}

${voffset 4}${font FontAwesome}${font}  Temperature ${alignr}${acpitemp} °C

${voffset 2}${font FontAwesome}${font}  /
${voffset 4}${fs_used /}/${fs_size /} ${alignr}${fs_free /} Free   ${fs_bar 8,80 /}

${voffset 2}${font FontAwesome}${font}  /home
${voffset 4}${fs_used /home}/${fs_size /home} ${alignr}${fs_free /home} Free   ${fs_bar 8,80 /home}

]]

