//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2619/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2619<F: Float>(t11697: F, t22153: F, t3577: F, t13969: F, t22274: F, t3515: F, t1227: F, t22196: F, t1222: F, t22015: F, t15740: F, t18584: F, t18965: F, t18997: F, t19077: F, t3447: F, t3578: F, t4733: F, t4889: F, t52903: F, t52995: F, t53087: F, t6219: F, t66545: F, t66554: F, t66566: F, t68513: F) -> F {
    let t73084 = t3577 * t11697 * t22153;
    let t73096 = t3515 * t13969 * t22274;
    let t73099 = t1227 * t13969 * t22196;
    let t73102 = t22015 * t1222;
    let t73108 = -t3577 * t3578 * t6219 * t4733 / F::cast_from(1536.0_f64) - t73084 / F::cast_from(2304.0_f64) - t15740 * t18584 / F::cast_from(768.0_f64) - t52903 * t18965 / F::cast_from(288.0_f64) + t4889 * t18997 / F::cast_from(36.0_f64) - t53087 * t19077 / F::cast_from(192.0_f64) - t66545 / F::cast_from(81.0_f64) - t73096 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t73099 + t66554 / F::cast_from(1536.0_f64) - t73102 / F::cast_from(288.0_f64) + t3447 * t52995 * t68513 / F::cast_from(12.0_f64) - t66566 / F::cast_from(2304.0_f64);
    t73108
}
