require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib 'target/release/embed'
  attach_function :process, [], :void
end

Hello.process

puts 'done!'