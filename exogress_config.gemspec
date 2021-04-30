require_relative 'lib/exogress_common/version'

Gem::Specification.new do |spec|
  spec.name          = "exogress_common"
  spec.version       = ExogressCommon::VERSION
  spec.authors       = ["Exogress Team"]
  spec.email         = ["team@exogress.com"]

  spec.summary = %q{Exogress config}
  spec.description = %q{Exogress config}
  spec.homepage = "https://exogress.com"
  spec.license = "Apache 2.0"
  spec.required_ruby_version = Gem::Requirement.new(">= 2.3.0")

  spec.metadata["allowed_push_host"] = "TODO: Set to 'http://mygemserver.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/exogress'"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files         = Dir.chdir(File.expand_path('..', __FILE__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  end
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_dependency 'rutie', '~> 0.0.4'
end
