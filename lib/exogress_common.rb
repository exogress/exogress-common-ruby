require "exogress_common/version"
require 'rutie'

class EntityException < StandardError
end

class ClientConfigException < StandardError
end

class ProjectConfigException < StandardError
end

class ParameterException < StandardError
end

module ExogressConfigRuby
  Rutie.new(:exogress_common).init 'Init_exogress_common', __dir__
end
