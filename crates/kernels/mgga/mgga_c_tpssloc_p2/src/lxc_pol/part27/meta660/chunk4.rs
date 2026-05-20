//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2309/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2309<F: Float>(t22704: F, t5336: F, t80798: F, t22724: F, t26436: F, t81037: F, t81041: F, t81043: F, t81047: F, t81050: F, t90865: F, t90867: F, t90868: F, t90873: F, t90876: F, t90883: F, t90887: F, t90889: F, t90892: F, t90895: F) -> F {
    let t90898 = t22704 * t80798 * t5336;
    let t90899 = F::cast_from(0.16449340668482264365e-1_f64) * t90898;
    let t90900 = t22724 * t26436;
    let t90902 = t90865 - t90867 + F::cast_from(0.63969658155208805863e-1_f64) * t90868 - F::cast_from(0.82246703342411321825e-2_f64) * t90873 - F::cast_from(0.19190897446562641759e-1_f64) * t81037 + t90876 + F::cast_from(0.19190897446562641759e-1_f64) * t81041 - F::cast_from(0.11514538467937585055e0_f64) * t81043 - F::cast_from(0.52089578783527170488e-1_f64) * t81047 + F::cast_from(0.82246703342411321824e-2_f64) * t81050 - F::cast_from(0.16449340668482264365e-1_f64) * t90883 - F::cast_from(0.82246703342411321825e-2_f64) * t90887 - t90889 - F::cast_from(0.3289868133696452873e-1_f64) * t90892 + F::cast_from(0.3289868133696452873e-1_f64) * t90895 - t90899 + F::cast_from(0.26044789391763585244e-1_f64) * t90900;
    t90902
}
