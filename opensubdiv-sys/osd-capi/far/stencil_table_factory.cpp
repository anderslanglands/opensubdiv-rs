#include <opensubdiv/far/stencilTableFactory.h>

class OpenSubdiv::Far::TopologyRefiner;
class OpenSubdiv::Far::StencilTable;

typedef OpenSubdiv::Far::StencilTableFactory StencilTableFactory;
typedef OpenSubdiv::Far::StencilTable StencilTable;
typedef OpenSubdiv::Far::StencilTableFactory::Options Options;
typedef OpenSubdiv::Far::TopologyRefiner TopologyRefiner;


extern "C" {
const StencilTable* StencilTableFactory_Create(TopologyRefiner* refiner, Options options) {
    return StencilTableFactory::Create(*refiner, options);
}
}