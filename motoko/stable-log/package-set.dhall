let upstream =
      https://github.com/dfinity/vessel-package-set/releases/download/mo-0.6.7-20210818/package-set.dhall

let Package =
      { name : Text, version : Text, repo : Text, dependencies : List Text }

in  upstream
