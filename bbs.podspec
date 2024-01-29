Pod::Spec.new do |s|
  s.name = "bbs"
  s.version = "0.0.1"
  s.summary = "Test"
  s.homepage = "https://google.ch"  
  s.author = "Jonas niestroj"
  s.license      = "MIT"
  s.platform     = :ios, "10.0"
  s.source       = { :http => 'file:' + __dir__ + '/' }

  s.source_files = "**/*.{h,m,swift}"
  s.requires_arc = true
  s.vendored_frameworks = 'bbs.xcframework'

end
