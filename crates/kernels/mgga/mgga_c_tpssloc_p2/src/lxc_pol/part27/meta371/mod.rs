//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1527;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1528;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta371<F: Float>(t13748: F, t973: F, t1611: F, t3088: F, t1036: F, t4617: F, t1023: F, t4347: F, t3071: F, t10422: F, t4574: F, t3070: F, t1597: F, t4509: F, t10237: F, t10189: F, t344: F, t4343: F, t2986: F, t134: F, t2978: F, t4338: F, t10190: F, t4514: F, t13528: F, t4510: F, t13532: F, t10213: F, t60: F, t13537: F, t10186: F, t10192: F, t10226: F, t10229: F, t4511: F, t4515: F, t4519: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13750, t13751, t13758, t13762, t13765, t13767) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1527::<F>(t13748, t973, t1611, t3088, t1036, t4617, t1023, t4347, t3071, t10422, t4574, t3070);
        let (t13770, t13782, t13783, t13787, t13788) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1528::<F>(t1597, t4509, t10237, t10189, t344, t4343, t2986, t134, t2978, t4338, t10190, t4514);
        let (t13797, t13804) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1529::<F>(t13788, t2986, t13528, t4510, t13532, t10213, t60, t344, t13537, t10186, t10192, t10226, t10229, t13770, t13782, t13787, t4511, t4515, t4519);
    (t13750, t13751, t13758, t13762, t13765, t13767, t13783, t13797, t13804)
}
