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
    ip: string;
    port: number;
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

export interface EtcdConfig {
    endpoints: string[];
    timeout: number;
    keepAlive: number;
    tls?: TlsOptions | undefined;
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
}

export interface MiddlewareConfig {
    name: string;
    headers?: HeadersConfig | undefined;
    protocol: string;
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

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updatedAt: string;
    createdAt: string;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    file_template?: boolean | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updated_at: string;
    version: number;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updated_at: Date;
    createdAt: Date;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate?: boo | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate?: string | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileemplate?: boolean | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updated_at: string;
    created_at: string;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updatedAt: Date;
    createdAt: Date;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate?: bo | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate?: b | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updatedAt: string;
    created_at: string;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updated_at: Date;
    created_at: Date;
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

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate: boolean;
    updated_at: string;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    fileTemplate?: boolean | undefined;
}

export interface TemplateInfo {
    name: string;
    path: string;
    description?: string | undefined;
    file_template: boolean;
}

export interface TlsOptions {
    domain?: string | undefined;
    cert?: string | undefined;
    key?: string | undefined;
    ca?: string | undefined;
}

export interface TraefikConfig {
    name?: string | undefined;
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
}

export interface TraefikConfig {
    name: OString;
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
}

export interface TraefikConfig {
    name?: string | undefined;
    description?: string | undefined;
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
}

export interface TraefikConfig {
    name: string;
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
}

export interface TraefikConfig {
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
}

export interface TraefikConfig {
    name: OpString;
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
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

