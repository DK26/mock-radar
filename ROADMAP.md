# Mock-Radar Development Roadmap

This document outlines the development priorities for Mock-Radar based on community needs, QRadar API usage patterns, and the value delivered to teams developing SOAR applications, CI/CD pipelines, and Detection as Code workflows.

## 🎯 **Phase 1: Core Functionality**

### 1. **Complete Reference Sets Implementation** 🚨 *Critical*
**Current Gap:** GET endpoint returns placeholder HTML instead of proper JSON response
**Business Value:** 
- ✅ Reference Sets are foundational to most QRadar integrations
- ✅ IOC management and threat intelligence workflows depend on this API
- ✅ Required for testing SOAR playbooks that manage indicators

### 2. **Offenses API Implementation** 🥇 *Highest Priority*
**Endpoints to Implement:**
- `GET /api/siem/offenses` - List offenses with filtering and pagination
- `GET /api/siem/offenses/{id}` - Retrieve detailed offense information
- `POST /api/siem/offenses/{id}/closing_reasons` - Close offenses with documented reasons
- `PUT /api/siem/offenses/{id}` - Update offense properties (assignment, notes, status)

**Business Value:**
- **SOAR Integration Foundation:** Offenses are the primary output of QRadar that triggers automated responses
- **Alert Management Testing:** Essential for validating incident response workflows
- **CI/CD Pipeline Testing:** Enables automated testing of alert processing and escalation logic
- **Detection Validation:** Confirms that detection rules produce expected offense structures

---

## 🎯 **Phase 2: Configuration Management**

### 3. **Log Sources API** 🥉
**Endpoints to Implement:**
- `GET /api/config/event_sources/log_source_management/log_sources`
- `POST /api/config/event_sources/log_source_management/log_sources`
- `PUT /api/config/event_sources/log_source_management/log_sources/{id}`
- `DELETE /api/config/event_sources/log_source_management/log_sources/{id}`

**Business Value:**
- **Infrastructure as Code:** Teams automate log source provisioning and configuration
- **Environment Consistency:** Ensures development and production environments have matching data sources
- **Onboarding Automation:** Streamlines the process of adding new data sources to QRadar
- **Testing Data Pipelines:** Validates that applications correctly configure data ingestion

### 4. **Custom Properties API** 🏅
**Endpoints to Implement:**
- `GET /api/config/event_sources/custom_properties/property_expressions`
- `POST /api/config/event_sources/custom_properties/property_expressions`
- `PUT /api/config/event_sources/custom_properties/property_expressions/{id}`
- `DELETE /api/config/event_sources/custom_properties/property_expressions/{id}`

**Business Value:**
- **Advanced Detection Logic:** Custom properties enable sophisticated detection rules based on parsed event data
- **Data Enrichment:** Supports extraction of custom fields from raw log data for analysis
- **Rule Development:** Essential for teams building complex detection logic that depends on custom event fields
- **Compliance Reporting:** Enables extraction of specific data points required for regulatory reporting

---

## 🎯 **Phase 3: Advanced Detection Capabilities**

### 5. **Rules API (Basic CRUD)** 🎖️ *Strategic Differentiator*
**Endpoints to Implement:**
- `GET /api/analytics/rules` - List detection rules with filtering capabilities
- `GET /api/analytics/rules/{id}` - Retrieve detailed rule configuration and metadata
- `PUT /api/analytics/rules/{id}` - Update rule properties (enable/disable, modify thresholds)

**Scope Limitations:** 
- CRUD operations for rule management without rule engine execution
- Rule validation and state management
- Metadata and configuration handling
- No actual rule processing or event correlation

**Business Value:**
- **Detection as Code:** Enables automated deployment and testing of detection rules
- **Rule Lifecycle Management:** Supports development workflows for security analysts
- **Configuration Testing:** Validates rule deployment processes and configuration management
- **Compliance Auditing:** Provides API access to rule configurations for audit and documentation

### 6. **Ariel Search API (Static Responses)** 🏆
**Endpoints to Implement:**
- `POST /api/ariel/searches` - Initiate searches with configurable response patterns
- `GET /api/ariel/searches/{search_id}` - Retrieve search status and execution details
- `GET /api/ariel/searches/{search_id}/results` - Access search results in QRadar format

**Scope Limitations:**
- Pre-configured response mappings based on search patterns
- Static result sets without AQL query execution
- Configurable response timing to simulate real search behavior
- No actual data processing or query optimization

**Business Value:**
- **Data Analysis Testing:** Enables testing of applications that perform QRadar searches
- **SOAR Enrichment:** Supports automated threat hunting and incident enrichment workflows
- **Reporting Automation:** Validates scheduled search and reporting functionality
- **Integration Testing:** Confirms proper handling of QRadar's asynchronous search model

---

## 🎯 **Future Considerations** (Phase 4+)

### **Additional QRadar APIs:**
- **Assets API:** Asset management and network topology integration
- **Custom Actions API:** Response automation and playbook integration
- **Administrative APIs:** User management and system configuration

### **Enhanced Capabilities:**
- **Multi-Version Support:** Compatibility with different QRadar API versions
- **Advanced AQL Simulation:** More sophisticated query-response mapping
- **Performance Benchmarking:** Response time and throughput optimization
- **Container Orchestration:** Kubernetes and Docker Compose deployment patterns

---

## 📊 **Success Metrics**

### **Technical Quality:**
- API endpoint coverage compared to QRadar Community Edition
- Response accuracy and format compliance
- Performance benchmarks and resource efficiency
- Test coverage and reliability metrics

### **Community Adoption:**
- Integration usage in CI/CD pipelines
- Community contributions and feedback
- Issue resolution and feature request fulfillment
- Documentation completeness and clarity

### **Ecosystem Impact:**
- SOAR platform integration examples
- Detection as Code workflow adoption
- Developer productivity improvements
- Industry recognition and adoption

---

## 🔄 **Roadmap Evolution**

This roadmap is updated quarterly based on:
- Community feedback and feature requests
- QRadar API changes and new releases
- Emerging patterns in SOAR and Detection as Code workflows
- Integration requirements from major security platforms

**Current Version:** 1.0  
**Last Updated:** July 14, 2025  
**Next Review:** October 14, 2025

---

## 📝 **Community Input**

We value community input for roadmap prioritization:
- **Feature Requests:** Submit issues with the `enhancement` label
- **Use Case Documentation:** Share your QRadar integration patterns
- **API Validation:** Help verify mock responses against real QRadar behavior
- **Testing Feedback:** Report accuracy issues or missing functionality

**[💬 Discuss Roadmap](https://github.com/DK26/mock-radar/discussions)**
