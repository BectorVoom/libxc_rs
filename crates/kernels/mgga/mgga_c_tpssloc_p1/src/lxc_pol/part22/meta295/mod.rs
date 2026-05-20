//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1456;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta295<F: Float>(t13784: F, t4338: F, t2986: F, t10190: F, t4514: F, t10213: F, t60: F, t344: F, t135: F, t340: F, t4548: F, t973: F, t2970: F, t4522: F, t10254: F, t3961: F, t10236: F, t10189: F, t1597: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13787, t13790, t13797, t13798, t13822) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1456::<F>(t13784, t4338, t2986, t10190, t4514, t10213, t60, t344, t135, t340);
        let (t13825, t13830, t13835, t13839, t13847) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1457::<F>(t13822, t4548, t973, t2970, t4522, t10254, t3961, t10236, t10189, t1597);
    (t13787, t13790, t13797, t13798, t13822, t13825, t13830, t13835, t13839, t13847)
}
