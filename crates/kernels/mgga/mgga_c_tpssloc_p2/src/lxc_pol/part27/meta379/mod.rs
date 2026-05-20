//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1554;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta379<F: Float>(t12606: F, t998: F, t974: F, t10868: F, t1539: F, t248: F, t1041: F, t1009: F, t4552: F, t1011: F, t1019: F, t1615: F, t3131: F, t1022: F, t883: F, t607: F, t3071: F, t360: F, t4342: F, t1025: F, t10403: F, t10413: F, t10909: F, t10923: F, t10927: F, t14174: F, t14180: F, t14184: F, t14189: F, t14194: F, t2960: F, t3070: F, t3117: F, t4590: F, t4609: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14198, t14202, t14203, t14205, t14206, t14207, t14211) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1554::<F>(t12606, t998, t974, t10868, t1539, t248, t1041, t1009, t4552, t1011, t1019, t1615, t3131);
        let (t14213, t14215, t14220, t14222, t14227, t14228, t14230, t14233) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1555::<F>(t1022, t883, t607, t14211, t3071, t1615, t360, t4342, t1025, t10403, t1041, t10413, t10909, t10923, t10927, t14174, t14180, t14184, t14189, t14194, t14198, t14203, t14207, t2960, t3070, t3117, t4590, t4609, t973);
    (t14198, t14202, t14205, t14206, t14213, t14215, t14220, t14222, t14227, t14228, t14230, t14233)
}
