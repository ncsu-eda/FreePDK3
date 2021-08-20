set files {
    nmos4.cdf
    pmos4.cdf
    nmos.cdf
    pmos.cdf
}

foreach f $files {
    puts "Loading $f"
    source [file dirname [info script]]/$f
}
