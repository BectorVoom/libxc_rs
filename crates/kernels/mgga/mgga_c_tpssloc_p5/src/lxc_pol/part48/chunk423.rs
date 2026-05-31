//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 423/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk423<F: Float>(t2924: F, t951: F, t941: F, t315: F, t323: F, t2906: F, t2786: F, t2789: F, t2796: F, t2839: F, t2847: F, t2853: F, t2856: F, t2861: F, t2863: F, t2881: F, t2886: F, t2889: F, t2898: F, t2900: F, t2905: F, t2907: F, t311: F, t924: F, t933: F, t943: F, t952: F) -> (F, F, F) {
    let t2925 = t2924 * t951;
    let t2928 = t941 * t941;
    let t2929 = F::cast_from(1.0_f64) / t2928;
    let t2930 = t315 * t2929;
    let t2931 = t323 * t323;
    let t2932 = F::cast_from(1.0_f64) / t2931;
    let t2933 = t2906 * t2932;
    let t2936 = -F::cast_from(0.310907e-1_f64) * t2853 * t311 + F::cast_from(2.0_f64) * t2856 * t933 - F::cast_from(2.0_f64) * t2861 * t2863 + F::cast_from(1.0_f64) * t924 * t2881 + F::cast_from(0.32163958997385070134e2_f64) * t2886 * t2889 + t2786 - t2789 + t2796 - t2839 - t2847 - F::cast_from(0.19751673498613801407e-1_f64) * t2898 + F::cast_from(0.11696447245269292414e1_f64) * t2900 * t952 - F::cast_from(0.11696447245269292414e1_f64) * t2905 * t2907 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t2925 + F::cast_from(0.17315859105681463759e2_f64) * t2930 * t2933;
    (t2929, t2932, t2936)
}
