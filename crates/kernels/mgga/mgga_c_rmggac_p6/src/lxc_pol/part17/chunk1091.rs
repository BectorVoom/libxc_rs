//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1091/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1091<F: Float>(t36913: F, t36916: F, t36922: F, t36925: F, t36936: F, t36943: F, t36948: F, t42087: F, t42101: F, t47831: F, t47833: F, t47835: F, t47840: F, t47845: F, t47855: F, t47857: F, t47861: F) -> F {
    let t47863 = -F::cast_from(0.5987120850931904282e-1_f64) * t47831 - F::cast_from(0.71845450211182851384e0_f64) * t47833 + F::cast_from(0.15965655602485078085e0_f64) * t47835 + t42087 - F::cast_from(0.31923449919973379548e-4_f64) * t47840 + F::cast_from(0.95770349759920138644e-4_f64) * t47845 - F::cast_from(0.59590439850616975158e-4_f64) * t42101 + F::cast_from(0.36021158228745895953e-3_f64) * t36913 + F::cast_from(0.33105799917009430643e-4_f64) * t36916 - F::cast_from(0.19211284388664477842e-2_f64) * t36922 - F::cast_from(0.72042316457491791906e-3_f64) * t36925 - F::cast_from(0.36021158228745895953e-3_f64) * t36936 + t36943 + F::cast_from(0.10248087766267884742e-3_f64) * t36948 - F::cast_from(0.42564599893297839398e-5_f64) * t47855 + F::cast_from(0.12769379967989351819e-4_f64) * t47857 + F::cast_from(0.12769379967989351819e-4_f64) * t47861;
    t47863
}
