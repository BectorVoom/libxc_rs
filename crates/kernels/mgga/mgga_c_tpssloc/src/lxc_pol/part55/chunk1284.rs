//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1284/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1284<F: Float>(t125295: F, t2122: F, t225: F, t34278: F, t117930: F, t118050: F, t118052: F, t1186: F, t1190: F, t1238: F, t1251: F, t1252: F, t14980: F, t1716: F, t2144: F, t2154: F, t24880: F, t27395: F, t27453: F, t27741: F, t27784: F, t27785: F, t32480: F, t32482: F, t32504: F, t32524: F, t34277: F, t34305: F, t34318: F, t3487: F, t3598: F, t460: F, t4945: F, t498: F, t5060: F, t7283: F, t7286: F, t7999: F, t8061: F, t8898: F, t94395: F) -> F {
    let t125596 = t2122 * t125295;
    let t125613 = t34278 * t225;
    let t125624 = F::new(2.0) * t3487 * t34318 + t1190 * t34277 * t498 - t14980 * t8898 + F::new(2.0) * t32482 * t5060 - F::new(0.14621636149762012769e-1) * t94395 * t32524 + F::new(0.54831135561607547883e-2) * t118050 - t4945 * t32480 + F::new(4.0) * t24880 * t8061 + F::new(0.16449340668482264365e-1) * t7283 * t1186 * t125596 + F::new(4.0) * t1238 * t3598 * t2154 * t27741 - F::new(0.43864908449286038307e-1) * t7999 * t32504 + F::new(2.0) * t1238 * t3598 * t34305 * t1251 - F::new(12.0) * t27784 * t27785 * t27395 - t125613 * t1252 + F::new(0.16449340668482264365e-1) * t7283 * t1716 * t117930 + F::new(0.10966227112321509577e-1) * t118052 - F::new(0.16449340668482264365e-1) * t7283 * t27453 * t460 * t2144 * t7286;
    t125624
}
