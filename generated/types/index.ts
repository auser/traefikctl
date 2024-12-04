export interface HostConfig {
    domain: string;
    paths: PathConfig[];
    deployments: Record<string, DeploymentConfig>;
    middlewares: string[];
    selection?: SelectionConfig | undefined;
}

export interface PartialEtcdConfig {
    endpoints?: string[] | undefined;
    timeout?: number | undefined;
    keepAlive?: number | undefined;
    tls?: TlsOptions | undefined;
}

export interface SelectionConfig {
    withCookie?: WithCookieConfig | undefined;
    fromClientIp?: FromClientIpConfig | undefined;
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

export interface HeadersConfig {
    customRequestHeaders: Record<string, string>;
    customResponseHeaders: Record<string, string>;
    accessControlAllowMethods: string[];
    accessControlAllowHeaders: string[];
    accessControlExposeHeaders: string[];
    accessControlAllowOriginList: string[];
    addVaryHeader: boolean;
}

export interface WithCookieConfig {
    name: string;
    value?: string | undefined;
}

export type DeploymentProtocol =
    | "Http"
    | "Https"
    | "Tcp"
    | "Invalid";

export interface MiddlewareConfig {
    name: string;
    headers?: HeadersConfig | undefined;
    protocol: string;
}

export interface PathConfig {
    path: string;
    deployments: Record<string, DeploymentConfig>;
    middlewares: string[];
    stripPrefix: boolean;
    passThrough: boolean;
}

export interface TraefikConfig {
    rulePrefix: string;
    etcd: EtcdConfig;
    hosts: HostConfig[];
    middlewares: Record<string, MiddlewareConfig>;
}

export interface EtcdConfig {
    endpoints: string[];
    timeout: number;
    keepAlive: number;
    tls?: TlsOptions | undefined;
}

export interface HealthCheckConfig {
    path: string;
    interval: string;
    timeout: string;
}

export interface FromClientIpConfig {
    range?: string | undefined;
    ip?: string | undefined;
}

export interface TlsOptions {
    domain?: string | undefined;
    cert?: string | undefined;
    key?: string | undefined;
    ca?: string | undefined;
}

