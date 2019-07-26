#include <opensubdiv/far/topologyRefiner.h>
#include <opensubdiv/sdc/options.h>
#include <opensubdiv/sdc/types.h>

typedef OpenSubdiv::Far::TopologyRefiner TopologyRefiner;
typedef OpenSubdiv::Far::TopologyLevel TopologyLevel;
typedef OpenSubdiv::Far::TopologyRefiner::UniformOptions UniformOptions;
typedef OpenSubdiv::Sdc::SchemeType SdcSchemeType;
typedef OpenSubdiv::Sdc::Options SdcOptions;

extern "C" {
void TopologyRefiner_destroy(TopologyRefiner* refiner) { delete refiner; }

void TopologyRefiner_RefineUniform(TopologyRefiner* refiner,
                                   UniformOptions options) {
    refiner->RefineUniform(options);
}

/// \brief Returns the subdivision scheme
SdcSchemeType TopologyRefiner_GetSchemeType(TopologyRefiner* refiner) {
    return refiner->GetSchemeType();
}

/// \brief Returns the subdivision options
SdcOptions TopologyRefiner_GetSchemeOptions(TopologyRefiner* refiner) {
    return refiner->GetSchemeOptions();
}

/// \brief Returns true if uniform refinement has been applied
bool TopologyRefiner_IsUniform(TopologyRefiner* refiner) {
    return refiner->IsUniform();
}

/// \brief Returns the number of refinement levels
int TopologyRefiner_GetNumLevels(TopologyRefiner* refiner) {
    return refiner->GetNumLevels();
}

/// \brief Returns the highest level of refinement
int TopologyRefiner_GetMaxLevel(TopologyRefiner* refiner) {
    return refiner->GetMaxLevel();
}

/// \brief Returns the maximum vertex valence in all levels
int TopologyRefiner_GetMaxValence(TopologyRefiner* refiner) {
    return refiner->GetMaxValence();
}

/// \brief Returns true if faces have been tagged as holes
bool TopologyRefiner_HasHoles(TopologyRefiner* refiner) {
    return refiner->HasHoles();
}

/// \brief Returns the total number of vertices in all levels
int TopologyRefiner_GetNumVerticesTotal(TopologyRefiner* refiner) {
    return refiner->GetNumVerticesTotal();
}

/// \brief Returns the total number of edges in all levels
int TopologyRefiner_GetNumEdgesTotal(TopologyRefiner* refiner) {
    return refiner->GetNumEdgesTotal();
}

/// \brief Returns the total number of edges in all levels
int TopologyRefiner_GetNumFacesTotal(TopologyRefiner* refiner) {
    return refiner->GetNumFacesTotal();
}

/// \brief Returns the total number of face vertices in all levels
int TopologyRefiner_GetNumFaceVerticesTotal(TopologyRefiner* refiner) {
    return refiner->GetNumFaceVerticesTotal();
}

/// \brief Returns a handle to access data specific to a particular level
const TopologyLevel* TopologyRefiner_GetLevel(TopologyRefiner* refiner, int level) {
    return &refiner->GetLevel(level);
}
}