/**
 * devolutions-gateway
 * Protocol-aware fine-grained relay server
 *
 * The version of the OpenAPI document: 2023.3.0
 * Contact: infos@devolutions.net
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */
import { PubKeyFormat } from './pubKeyFormat';
import { DataEncoding } from './dataEncoding';


export interface SubProvisionerKey { 
    Encoding?: DataEncoding | null;
    Format?: PubKeyFormat | null;
    /**
     * The key ID for this subkey
     */
    Id: string;
    /**
     * The binary-to-text-encoded key data
     */
    Value: string;
}

