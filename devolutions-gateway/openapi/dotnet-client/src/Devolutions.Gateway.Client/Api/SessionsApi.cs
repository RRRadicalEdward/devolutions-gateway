/*
 * devolutions-gateway
 *
 * Protocol-aware fine-grained relay server
 *
 * The version of the OpenAPI document: 2023.2.1
 * Contact: infos@devolutions.net
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Net;
using System.Net.Mime;
using Devolutions.Gateway.Client.Client;
using Devolutions.Gateway.Client.Model;

namespace Devolutions.Gateway.Client.Api
{

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface ISessionsApiSync : IApiAccessor
    {
        #region Synchronous Operations
        /// <summary>
        /// Lists running sessions
        /// </summary>
        /// <remarks>
        /// Lists running sessions
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>List&lt;SessionInfo&gt;</returns>
        List<SessionInfo> GetSessions(int operationIndex = 0);

        /// <summary>
        /// Lists running sessions
        /// </summary>
        /// <remarks>
        /// Lists running sessions
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of List&lt;SessionInfo&gt;</returns>
        ApiResponse<List<SessionInfo>> GetSessionsWithHttpInfo(int operationIndex = 0);
        /// <summary>
        /// Terminate forcefully a running session
        /// </summary>
        /// <remarks>
        /// Terminate forcefully a running session
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns></returns>
        void TerminateSession(Guid id, int operationIndex = 0);

        /// <summary>
        /// Terminate forcefully a running session
        /// </summary>
        /// <remarks>
        /// Terminate forcefully a running session
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of Object(void)</returns>
        ApiResponse<Object> TerminateSessionWithHttpInfo(Guid id, int operationIndex = 0);
        #endregion Synchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface ISessionsApiAsync : IApiAccessor
    {
        #region Asynchronous Operations
        /// <summary>
        /// Lists running sessions
        /// </summary>
        /// <remarks>
        /// Lists running sessions
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of List&lt;SessionInfo&gt;</returns>
        System.Threading.Tasks.Task<List<SessionInfo>> GetSessionsAsync(int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));

        /// <summary>
        /// Lists running sessions
        /// </summary>
        /// <remarks>
        /// Lists running sessions
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (List&lt;SessionInfo&gt;)</returns>
        System.Threading.Tasks.Task<ApiResponse<List<SessionInfo>>> GetSessionsWithHttpInfoAsync(int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));
        /// <summary>
        /// Terminate forcefully a running session
        /// </summary>
        /// <remarks>
        /// Terminate forcefully a running session
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of void</returns>
        System.Threading.Tasks.Task TerminateSessionAsync(Guid id, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));

        /// <summary>
        /// Terminate forcefully a running session
        /// </summary>
        /// <remarks>
        /// Terminate forcefully a running session
        /// </remarks>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse</returns>
        System.Threading.Tasks.Task<ApiResponse<Object>> TerminateSessionWithHttpInfoAsync(Guid id, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));
        #endregion Asynchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface ISessionsApi : ISessionsApiSync, ISessionsApiAsync
    {

    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public partial class SessionsApi : ISessionsApi
    {
        private Devolutions.Gateway.Client.Client.ExceptionFactory _exceptionFactory = (name, response) => null;

        /// <summary>
        /// Initializes a new instance of the <see cref="SessionsApi"/> class.
        /// </summary>
        /// <returns></returns>
        public SessionsApi() : this((string)null)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="SessionsApi"/> class.
        /// </summary>
        /// <returns></returns>
        public SessionsApi(string basePath)
        {
            this.Configuration = Devolutions.Gateway.Client.Client.Configuration.MergeConfigurations(
                Devolutions.Gateway.Client.Client.GlobalConfiguration.Instance,
                new Devolutions.Gateway.Client.Client.Configuration { BasePath = basePath }
            );
            this.Client = new Devolutions.Gateway.Client.Client.ApiClient(this.Configuration.BasePath);
            this.AsynchronousClient = new Devolutions.Gateway.Client.Client.ApiClient(this.Configuration.BasePath);
            this.ExceptionFactory = Devolutions.Gateway.Client.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="SessionsApi"/> class
        /// using Configuration object
        /// </summary>
        /// <param name="configuration">An instance of Configuration</param>
        /// <returns></returns>
        public SessionsApi(Devolutions.Gateway.Client.Client.Configuration configuration)
        {
            if (configuration == null) throw new ArgumentNullException("configuration");

            this.Configuration = Devolutions.Gateway.Client.Client.Configuration.MergeConfigurations(
                Devolutions.Gateway.Client.Client.GlobalConfiguration.Instance,
                configuration
            );
            this.Client = new Devolutions.Gateway.Client.Client.ApiClient(this.Configuration.BasePath);
            this.AsynchronousClient = new Devolutions.Gateway.Client.Client.ApiClient(this.Configuration.BasePath);
            ExceptionFactory = Devolutions.Gateway.Client.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="SessionsApi"/> class
        /// using a Configuration object and client instance.
        /// </summary>
        /// <param name="client">The client interface for synchronous API access.</param>
        /// <param name="asyncClient">The client interface for asynchronous API access.</param>
        /// <param name="configuration">The configuration object.</param>
        public SessionsApi(Devolutions.Gateway.Client.Client.ISynchronousClient client, Devolutions.Gateway.Client.Client.IAsynchronousClient asyncClient, Devolutions.Gateway.Client.Client.IReadableConfiguration configuration)
        {
            if (client == null) throw new ArgumentNullException("client");
            if (asyncClient == null) throw new ArgumentNullException("asyncClient");
            if (configuration == null) throw new ArgumentNullException("configuration");

            this.Client = client;
            this.AsynchronousClient = asyncClient;
            this.Configuration = configuration;
            this.ExceptionFactory = Devolutions.Gateway.Client.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// The client for accessing this underlying API asynchronously.
        /// </summary>
        public Devolutions.Gateway.Client.Client.IAsynchronousClient AsynchronousClient { get; set; }

        /// <summary>
        /// The client for accessing this underlying API synchronously.
        /// </summary>
        public Devolutions.Gateway.Client.Client.ISynchronousClient Client { get; set; }

        /// <summary>
        /// Gets the base path of the API client.
        /// </summary>
        /// <value>The base path</value>
        public string GetBasePath()
        {
            return this.Configuration.BasePath;
        }

        /// <summary>
        /// Gets or sets the configuration object
        /// </summary>
        /// <value>An instance of the Configuration</value>
        public Devolutions.Gateway.Client.Client.IReadableConfiguration Configuration { get; set; }

        /// <summary>
        /// Provides a factory method hook for the creation of exceptions.
        /// </summary>
        public Devolutions.Gateway.Client.Client.ExceptionFactory ExceptionFactory
        {
            get
            {
                if (_exceptionFactory != null && _exceptionFactory.GetInvocationList().Length > 1)
                {
                    throw new InvalidOperationException("Multicast delegate for ExceptionFactory is unsupported.");
                }
                return _exceptionFactory;
            }
            set { _exceptionFactory = value; }
        }

        /// <summary>
        /// Lists running sessions Lists running sessions
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>List&lt;SessionInfo&gt;</returns>
        public List<SessionInfo> GetSessions(int operationIndex = 0)
        {
            Devolutions.Gateway.Client.Client.ApiResponse<List<SessionInfo>> localVarResponse = GetSessionsWithHttpInfo();
            return localVarResponse.Data;
        }

        /// <summary>
        /// Lists running sessions Lists running sessions
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of List&lt;SessionInfo&gt;</returns>
        public Devolutions.Gateway.Client.Client.ApiResponse<List<SessionInfo>> GetSessionsWithHttpInfo(int operationIndex = 0)
        {
            Devolutions.Gateway.Client.Client.RequestOptions localVarRequestOptions = new Devolutions.Gateway.Client.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }


            localVarRequestOptions.Operation = "SessionsApi.GetSessions";
            localVarRequestOptions.OperationIndex = operationIndex;

            // authentication (scope_token) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<List<SessionInfo>>("/jet/sessions", localVarRequestOptions, this.Configuration);
            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("GetSessions", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Lists running sessions Lists running sessions
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of List&lt;SessionInfo&gt;</returns>
        public async System.Threading.Tasks.Task<List<SessionInfo>> GetSessionsAsync(int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {
            Devolutions.Gateway.Client.Client.ApiResponse<List<SessionInfo>> localVarResponse = await GetSessionsWithHttpInfoAsync(operationIndex, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Lists running sessions Lists running sessions
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (List&lt;SessionInfo&gt;)</returns>
        public async System.Threading.Tasks.Task<Devolutions.Gateway.Client.Client.ApiResponse<List<SessionInfo>>> GetSessionsWithHttpInfoAsync(int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {

            Devolutions.Gateway.Client.Client.RequestOptions localVarRequestOptions = new Devolutions.Gateway.Client.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }


            localVarRequestOptions.Operation = "SessionsApi.GetSessions";
            localVarRequestOptions.OperationIndex = operationIndex;

            // authentication (scope_token) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = await this.AsynchronousClient.GetAsync<List<SessionInfo>>("/jet/sessions", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("GetSessions", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Terminate forcefully a running session Terminate forcefully a running session
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns></returns>
        public void TerminateSession(Guid id, int operationIndex = 0)
        {
            TerminateSessionWithHttpInfo(id);
        }

        /// <summary>
        /// Terminate forcefully a running session Terminate forcefully a running session
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of Object(void)</returns>
        public Devolutions.Gateway.Client.Client.ApiResponse<Object> TerminateSessionWithHttpInfo(Guid id, int operationIndex = 0)
        {
            Devolutions.Gateway.Client.Client.RequestOptions localVarRequestOptions = new Devolutions.Gateway.Client.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
            };

            var localVarContentType = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.PathParameters.Add("id", Devolutions.Gateway.Client.Client.ClientUtils.ParameterToString(id)); // path parameter

            localVarRequestOptions.Operation = "SessionsApi.TerminateSession";
            localVarRequestOptions.OperationIndex = operationIndex;

            // authentication (scope_token) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Post<Object>("/jet/session/{id}/terminate", localVarRequestOptions, this.Configuration);
            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("TerminateSession", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Terminate forcefully a running session Terminate forcefully a running session
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of void</returns>
        public async System.Threading.Tasks.Task TerminateSessionAsync(Guid id, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {
            await TerminateSessionWithHttpInfoAsync(id, operationIndex, cancellationToken).ConfigureAwait(false);
        }

        /// <summary>
        /// Terminate forcefully a running session Terminate forcefully a running session
        /// </summary>
        /// <exception cref="Devolutions.Gateway.Client.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id">Session / association ID of the session to terminate</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse</returns>
        public async System.Threading.Tasks.Task<Devolutions.Gateway.Client.Client.ApiResponse<Object>> TerminateSessionWithHttpInfoAsync(Guid id, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {

            Devolutions.Gateway.Client.Client.RequestOptions localVarRequestOptions = new Devolutions.Gateway.Client.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
            };

            var localVarContentType = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = Devolutions.Gateway.Client.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.PathParameters.Add("id", Devolutions.Gateway.Client.Client.ClientUtils.ParameterToString(id)); // path parameter

            localVarRequestOptions.Operation = "SessionsApi.TerminateSession";
            localVarRequestOptions.OperationIndex = operationIndex;

            // authentication (scope_token) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = await this.AsynchronousClient.PostAsync<Object>("/jet/session/{id}/terminate", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("TerminateSession", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

    }
}
