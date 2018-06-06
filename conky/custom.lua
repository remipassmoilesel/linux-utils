
-- print("debug")

require 'cairo'

custom_namespace = {
    public_address = nil,
}

function conky_command(command)
    local commandOutput = io.popen(command)
    return commandOutput:read('*a')
end

function conky_public_address()
    if (custom_namespace['public_address'] == nil) then
        local public_address = conky_command("curl ipinfo.io/ip"):sub(0, -2)
        print(public_address)
        custom_namespace['public_address'] = public_address
    end
    return custom_namespace['public_address']
end

function conky_private_addresses()
    local command = "ip a | grep -Eo 'inet (addr:)?([0-9]*\\.){3}[0-9]*' | grep -Eo '([0-9]*\\.){3}[0-9]*' | grep -v '127.0.0.1'"
    local addressesStr = conky_command(command):sub(0, -2)
    local addresses = string.gmatch(addressesStr, "[%d%p]+")
    local filteredAddresses = {}
    for addr in addresses do
        if addr:sub(0, 3) ~= "172" then
            table.insert(filteredAddresses, addr)
        end
    end
    return table.concat(filteredAddresses, ', ')
end