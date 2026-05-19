//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 957/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk957<F: Float>(t2001: F, t2281: F, t305: F, t551: F, t7720: F, t1364: F, t35514: F, t39899: F, t39901: F, t39923: F, t39927: F, t45864: F, t45866: F, t45869: F, t45872: F, t45874: F, t45880: F, t45884: F, t5055: F, t6421: F, t6441: F, t6473: F, t665: F, t8393: F, t8396: F, t903: F) -> F {
    let t45889 = t2001 * t305 * t2281 * t551;
    let t45890 = t7720 * t45889;
    let t45892 = F::cast_from(0.17961362552795712846e0_f64) * t903 * t665 * t6441 - F::cast_from(0.23948483403727617128e0_f64) * t1364 * t665 * t6421 - F::cast_from(0.54549323308490683458e-1_f64) * t39899 + F::cast_from(0.51077519871957407276e-4_f64) * t45864 + F::cast_from(0.85129199786595678796e-5_f64) * t45866 + F::cast_from(0.19863479950205658386e-3_f64) * t39901 - F::cast_from(0.42564599893297839398e-5_f64) * t45869 + F::cast_from(0.33105799917009430643e-4_f64) * t35514 + F::cast_from(0.12769379967989351819e-4_f64) * t45872 - F::cast_from(0.12769379967989351819e-4_f64) * t45874 + F::cast_from(0.35922725105591425692e0_f64) * t5055 * t8393 - F::cast_from(0.47896966807455234256e0_f64) * t6473 * t8396 - F::cast_from(0.13637330827122670864e-1_f64) * t45880 - F::cast_from(0.13637330827122670864e-1_f64) * t45884 - F::new(0.4726e1) * t39923 - t39927 - F::cast_from(0.25538759935978703638e-4_f64) * t45890;
    t45892
}
