#include <opensubdiv/far/primvarRefiner.h>
#include <opensubdiv/sdc/options.h>
#include <opensubdiv/sdc/types.h>

#include <stdio.h>

typedef OpenSubdiv::Far::PrimvarRefiner PrimvarRefiner;
typedef OpenSubdiv::Far::TopologyRefiner TopologyRefiner;

template <int N> struct Primvar {
    float v[N];

    void Clear() {
        for (int i = 0; i < N; ++i) {
            v[i] = 0.0f;
        }
    }

    void AddWithWeight(const Primvar& p, float weight) {
        for (int i = 0; i < N; ++i) {
            v[i] += p.v[i] * weight;
        }
    }
};

extern "C" {
PrimvarRefiner* PrimvarRefiner_create(TopologyRefiner* tr) {
    return new PrimvarRefiner(*tr);
}

void PrimvarRefiner_destroy(PrimvarRefiner* pr) { delete pr; }

const TopologyRefiner* PrimvarRefiner_GetTopologyRefiner(PrimvarRefiner* pr) {
    return &pr->GetTopologyRefiner();
}

void PrimvarRefiner_Interpolate(PrimvarRefiner* pr, int num_elements, int level,
                                float* src, float* dst) {
    Primvar<3>* dst3;
    switch (num_elements) {
    case 3:
        dst3 = (Primvar<3>*)dst;
        pr->Interpolate(level, (Primvar<3>*)src, dst3);
        break;
    default:
        printf("Invalid num elements for Interpolate: %d\n", num_elements);
        std::terminate();
    }
}

void PrimvarRefiner_InterpolateVarying(PrimvarRefiner* pr, int num_elements,
                                       int level, float* src, float* dst) {
    Primvar<3>* dst3;
    switch (num_elements) {
    case 3:
        dst3 = (Primvar<3>*)dst;
        pr->InterpolateVarying(level, (Primvar<3>*)src, dst3);
        break;
    default:
        printf("Invalid num elements for Interpolate: %d\n", num_elements);
        std::terminate();
    }
}

void PrimvarRefiner_InterpolateFaceUniform(PrimvarRefiner* pr, int num_elements,
                                           int level, float* src, float* dst) {
    Primvar<3>* dst3;
    switch (num_elements) {
    case 3:
        dst3 = (Primvar<3>*)dst;
        pr->InterpolateFaceUniform(level, (Primvar<3>*)src, dst3);
        break;
    default:
        printf("Invalid num elements for Interpolate: %d\n", num_elements);
        std::terminate();
    }
}

void PrimvarRefiner_InterpolateFaceVarying(PrimvarRefiner* pr, int num_elements,
                                           int level, float* src, float* dst) {
    Primvar<3>* dst3;
    switch (num_elements) {
    case 3:
        dst3 = (Primvar<3>*)dst;
        pr->InterpolateFaceVarying(level, (Primvar<3>*)src, dst3);
        break;
    default:
        printf("Invalid num elements for Interpolate: %d\n", num_elements);
        std::terminate();
    }
}
}