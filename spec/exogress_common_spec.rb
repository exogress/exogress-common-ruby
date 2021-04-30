RSpec.describe ExogressConfig do
  it "has a version number" do
    expect(ExogressConfig::VERSION).not_to be nil
  end

  it "raises exception on error" do
    expect { ClientConfig.new(<<EOF) }.to raise_error ClientConfigException
revision: 1
EOF
  end

  it "correctly parse config" do
    ClientConfig.new(<<EOF)
version: 1.0.0
revision: 1
name: develop
mount_points:
  main:
    handlers:
      main:
        type: proxy
        priority: 50
        upstream: backend
      static:
        type: static_dir
        dir: ./path
        priority: 30
EOF
  end
end
