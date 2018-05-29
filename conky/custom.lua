
-- print("debug")

require 'cairo'

custom_namespace = {
    public_address = nil,
}

function conky_public_address()
    if (custom_namespace['public_address'] == nil) then
        local curl_command = io.popen("curl ipinfo.io/ip")
        local public_address = curl_command:read()
        custom_namespace['public_address'] = public_address
    end
    return custom_namespace['public_address']
end
