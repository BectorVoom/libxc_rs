//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 889/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk889<F: Float>(t356: F, t638: F, t639: F, t9745: F, t34705: F, t34707: F, t34711: F, t38676: F, t38705: F, t38710: F, t38712: F, t44886: F, t44888: F, t44891: F, t44894: F, t44901: F, t44906: F, t44909: F, t44911: F, t44916: F) -> F {
    let t44920 = t638 * t639 * t9745 * t356;
    let t44922 = F::cast_from(0.12414674968878536491e-4_f64) * t44886 - F::cast_from(0.19863479950205658386e-4_f64) * t44888 - t38676 + t34705 + t34707 - t34711 + F::cast_from(0.72042316457491791906e-3_f64) * t44891 - F::cast_from(0.10248087766267884742e-3_f64) * t44894 + t38705 - F::cast_from(0.23836175940246790063e-3_f64) * t38710 - F::cast_from(0.59590439850616975157e-4_f64) * t38712 - F::cast_from(0.31923449919973379548e-4_f64) * t44901 - F::cast_from(0.31923449919973379548e-4_f64) * t44906 - F::cast_from(0.99317399751028291929e-5_f64) * t44909 - F::cast_from(0.27274661654245341728e-1_f64) * t44911 + F::cast_from(0.15243824895787514157e-3_f64) * t44916 + F::cast_from(0.15243824895787514157e-3_f64) * t44920;
    t44922
}
