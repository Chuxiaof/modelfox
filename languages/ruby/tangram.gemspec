Gem::Specification.new do |s|
  s.name = "tangram"
  s.version = "0.6.1"
  s.summary = "Tangram for Ruby"
  s.description = "Make predictions with a Tangram model from your Ruby app. Learn more at https://www.tangram.dev/."
  s.authors = ["Tangram"]
  s.email = "help@tangram.dev"
  s.files = Dir["**/**"].grep_v(/.gem$/)
  s.homepage = "https://rubygems.org/gems/tangram"
  s.license = "MIT"
  s.add_dependency "ffi", "~> 1"
end
