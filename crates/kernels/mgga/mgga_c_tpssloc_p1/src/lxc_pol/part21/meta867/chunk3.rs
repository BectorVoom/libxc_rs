//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3167/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3167<F: Float>(t11818: F, t248: F, t3506: F, t6225: F, t1174: F, t11825: F, t1214: F, t1227: F, t1230: F, t15672: F, t15761: F, t1737: F, t19026: F, t19051: F, t3440: F, t3496: F, t3511: F, t3515: F, t3518: F, t3587: F, t475: F, t4889: F, t5024: F, t52568: F, t6211: F, t63311: F, t63353: F, t65264: F, t65528: F, t65541: F, t65545: F, t65552: F, t65554: F) -> F {
    let t65558 = t3506 * t248 * t11818 * t6225;
    let t65565 = -t1227 * t248 * t1230 * t63353 / F::new(4608.0) - t11825 * t6211 / F::new(2304.0) - t65528 / F::new(13824.0) + t52568 * t1737 / F::new(1536.0) - t3515 * t248 * t1214 * t65264 * t475 / F::new(1536.0) + F::new(19.0) / F::new(1728.0) * t19026 * t3496 + F::new(19.0) / F::new(864.0) * t65541 * t3511 - F::new(19.0) / F::new(1728.0) * t65545 * t3518 + t5024 * t15761 / F::new(432.0) + F::new(5.0) / F::new(13824.0) * t19051 * t3587 + t65552 / F::new(10368.0) + t65554 / F::new(2304.0) - t65558 / F::new(6912.0) - F::new(4.0) / F::new(81.0) * t4889 * t15672 + t1174 * t3440 * t63311 / F::new(108.0);
    t65565
}
