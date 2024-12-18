export interface ConfigVersionHistory {
    id: number;
    configId: number;
    name: string;
    config: Record<any, any>;
    createdAt: Date;
    version: number;
}

export interface DeploymentConfig {
    name: string;
    target: DeploymentTarget;
    weight: number;
    selection?: SelectionConfig | undefined;
    protocol: DeploymentProtocol;
    middlewares?: string[] | undefined;
}

export enum DeploymentProtocol {
    HTTP = "Http",
    HTTPS = "Https",
    TCP = "Tcp",
    INVALID = "Invalid",
}

export enum DeploymentTarget {
    IP_AND_PORT = "IpAndPort",
    SERVICE = "Service",
}

export interface EtcdConfig {
    endpoints: string[];
    timeout: number;
    keepAlive: number;
    tls?: TlsOptions | undefined;
}

export interface ForwardAuthConfig {
    address?: string | undefined;
    trustForwardHeader?: boolean | undefined;
    authResponseHeaders?: string[] | undefined;
    authResponseHeadersRegex?: string | undefined;
}

export interface FromClientIpConfig {
    range?: string | undefined;
    ip?: string | undefined;
}

export interface HeadersConfig {
    customRequestHeaders: Record<string, string>;
    customResponseHeaders: Record<string, string>;
    accessControlAllowMethods: string[];
    accessControlAllowHeaders: string[];
    accessControlExposeHeaders: string[];
    accessControlAllowOriginList: string[];
    addVaryHeader: boolean;
}

export interface HealthCheckConfig {
    path: string;
    interval: string;
    timeout: string;
}

export interface HostConfig {
    domain: string;
    paths: PathConfig[];
    deployments: Record<string, DeploymentConfig>;
    middlewares: string[];
    selection?: SelectionConfig | undefined;
    forwardHost: boolean;
}

export interface IPAndPort {
    ip: string;
    port: number;
}

export interface MiddlewareConfig {
    name: string;
    headers?: HeadersConfig | undefined;
    protocol: string;
    forwardAuth?: ForwardAuthConfig | undefined;
}

export interface PartialEtcdConfig {
    endpoints?: string[] | undefined;
    timeout?: number | undefined;
    keepAlive?: number | undefined;
    tls?: TlsOptions | undefined;
}

export interface PathConfig {
    path: string;
    deployments: Record<string, DeploymentConfig>;
    middlewares: string[];
    stripPrefix: boolean;
    passThrough: boolean;
}

export interface SelectionConfig {
    withCookie?: WithCookieConfig | undefined;
    fromClientIp?: FromClientIpConfig | undefined;
}

export interface ServiceConfig {
    name: string;
    deployment: DeploymentConfig;
    passHostHeader: boolean;
}

export interface TemplateInfo {
    id: number;
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updated_at: Date;
    created_at: Date;
}

export interface TlsOptions {
    domain?: string | undefined;
    cert?: string | undefined;
    key?: string | undefined;
    ca?: string | undefined;
}

export interface TraefikConfig {
    name?: string | undefined;
    description?: string | undefined;
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
    services?: Record<string, ServiceConfig> | undefined;
}

export interface TraefikConfigVersion {
    id: number;
    name: string;
    config: Record<any, any>;
    createdAt: Date;
    updatedAt: Date;
    version: number;
}

export interface WithCookieConfig {
    name: string;
    value?: string | undefined;
}

