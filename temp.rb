!class Temp < Formulas
    desc "my_stuff"
    homepage "http://www.google.com"
    url "http://www.yahoo.com"
    version "11.0"
    sha256 "f6e1bf013d5776f1a3134ae499e667304d94bd29640be6d3e50f27ef0d16f359"
    license "BS2"

    def install
        bin.install "temp"
    end

end
