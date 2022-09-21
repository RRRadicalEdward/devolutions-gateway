/*
 * devolutions-gateway
 *
 * Protocol-aware fine-grained relay server
 *
 * The version of the OpenAPI document: 2022.3.0
 * Contact: infos@devolutions.net
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using System.Text.RegularExpressions;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System.ComponentModel.DataAnnotations;
using OpenAPIDateConverter = Devolutions.Gateway.Client.Client.OpenAPIDateConverter;

namespace Devolutions.Gateway.Client.Model
{
    /// <summary>
    /// Defines PubKeyFormat
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum PubKeyFormat
    {
        /// <summary>
        /// Enum Spki for value: Spki
        /// </summary>
        [EnumMember(Value = "Spki")]
        Spki = 1,

        /// <summary>
        /// Enum Rsa for value: Rsa
        /// </summary>
        [EnumMember(Value = "Rsa")]
        Rsa = 2

    }

    public static class PubKeyFormatExtensions
    {
        /// <summary>
        /// Returns the value as string for a given variant
        /// </summary>
        public static string ToValue(this PubKeyFormat variant)
        {
            switch (variant)
            {
                case PubKeyFormat.Spki:
                    return "Spki";
                case PubKeyFormat.Rsa:
                    return "Rsa";
                default:
                    throw new ArgumentOutOfRangeException(nameof(variant), $"Unexpected variant: {variant}");
            }
        }
    }

}
