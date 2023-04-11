/**
 * devolutions-gateway
 * Protocol-aware fine-grained relay server
 *
 * The version of the OpenAPI document: 2023.1.3
 * Contact: infos@devolutions.net
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */
import { ListenerUrls } from './listenerUrls';


/**
 * Service configuration diagnostic
 */
export interface ConfigDiagnostic { 
    /**
     * This Gateway\'s hostname
     */
    hostname: string;
    /**
     * This Gateway\'s unique ID
     */
    id?: string;
    /**
     * Listeners configured on this instance
     */
    listeners: Array<ListenerUrls>;
    /**
     * Gateway service version
     */
    version: string;
}

