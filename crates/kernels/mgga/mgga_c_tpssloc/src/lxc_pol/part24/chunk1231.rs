//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1231/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1231<F: Float>(t3173: F, t3175: F, t1921: F, t1054: F, t3206: F, t1920: F, t23353: F, t968: F, t1049: F, t23592: F, t10164: F, t225: F, t23384: F, t23595: F, t10166: F, t10181: F, t1052: F, t1065: F, t11084: F, t1955: F, t1956: F, t23327: F, t23329: F, t23341: F, t23354: F, t23394: F, t23588: F, t23594: F, t23721: F, t3010: F, t3016: F, t3026: F, t3174: F, t43440: F, t43604: F, t6680: F, t6687: F, t6704: F, t6705: F, t884: F, t986: F) -> (F, F, F) {
    let t82441 = t3173 * t3175;
    let t82442 = t1921 * t82441;
    let t82457 = t1054 * t3206;
    let t82463 = t1920 * t968 * t23353;
    let t82469 = t23592 * t1049;
    let t82481 = t225 * t10164;
    let t82490 = t23384 * t23595;
    let t82492 = -0.49348022005446793095e-1 * t6687 * t986 * t82442 + 0.24674011002723396548e-1 * t6687 * t3016 * t23588 + 6.0 * t1052 * t3174 * t23721 * t1065 + 0.49348022005446793095e-1 * t6687 * t6704 * t23394 * t10181 - 0.82246703342411321826e-2 * t23327 * t23329 * t82457 * t884 + 0.82246703342411321826e-2 * t82463 + 24.0 * t1052 * t43604 * t1955 * t10166 + 0.10966227112321509577e-1 * t6687 * t82469 * t23594 + 0.24674011002723396548e-1 * t6687 * t3010 * t23588 - 0.65797362673929057459e-1 * t6680 * t23354 - 18.0 * t3026 * t23341 - t43440 * t1956 - 0.49348022005446793095e-1 * t6687 * t6704 * t82481 * t10166 - 0.82246703342411321825e-2 * t6687 * t6704 * t6705 * t11084 + 0.36554090374405031922e-2 * t82490;
    (t82441, t82457, t82492)
}
