# Activer le support USB dans Virtual Box

S'ajouter au groupe vboxusers

    # Nécessite une reconnexion pour prendre effet
    $ sudo usermod -aG vboxusers remipassmoilesel


Pour une utilisation permanente, dans virtual box:

    Go to Settings for the VM, Go to USB.
    
    Click USB+ Icon and select device from List.
    
    This will add a Filter Spec to the List, repeat as necessary for more devices.
    
    Start VM - Devices should appear in Guest OS, and mount or request drivers etc. as applicable for the OS.
    
    Devices Mounted in Guest OS will be unavailable in Host OS.

Pour une utilisation temporaire:

    Insert USB Device and Wait for it to activate in Host OS.
    
    Start or go to VM.
    
    Right Click USB Icon in the VM Status Bar at bottom of screen, OR from Menu go Devices > USB Devices, and select desired device.
    
    Devices should appear in Guest OS, and mount or request drivers etc as normal for the OS. 

Source: https://help.ubuntu.com/community/VirtualBox/USB
