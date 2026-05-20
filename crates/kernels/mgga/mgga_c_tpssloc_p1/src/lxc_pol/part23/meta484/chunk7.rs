//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1479/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1479<F: Float>(t11606: F, t11881: F, t11883: F, t11888: F, t11889: F, t1238: F, t1241: F, t1244: F, t1246: F, t15027: F, t15245: F, t1720: F, t1751: F, t1758: F, t1761: F, t19201: F, t19232: F, t19249: F, t22008: F, t22114: F, t22243: F, t22327: F, t22341: F, t22354: F, t22361: F, t22365: F, t22372: F, t22386: F, t22390: F, t22394: F, t3610: F, t3612: F, t3624: F, t44698: F, t44701: F, t44724: F, t44726: F, t45350: F, t466: F, t491: F, t494: F, t4945: F, t498: F, t5055: F, t5064: F, t53565: F, t6168: F, t6218: F, t6238: F, t6243: F, t6244: F, t6252: F, t6261: F, t6263: F, t6265: F, t6267: F, t6268: F, t65262: F, t73613: F, t73856: F, t73891: F, t79008: F, t79260: F, t79391: F, t79398: F, t79410: F, t79453: F, t79461: F, t79467: F, t79473: F, t79524: F) -> F {
    let t79533 = -F::new(36.0) * t1238 * t11606 * t6243 * t6267 - F::new(4.0) * t73613 * t1761 - F::new(4.0) * t4945 * t22394 - F::new(12.0) * t73856 * t1761 + F::new(4.0) * t1720 * t22327 * t498 + t466 * t79391 * t498 + F::new(12.0) * t19249 * t6244 - F::new(6.0) * t19249 * t6268 + F::new(24.0) * t1238 * t45350 * t79398 + F::new(12.0) * t19232 * t6244 - t1238 * t1241 * (F::new(36.0) * t11881 * t6252 * t11883 * t6218 + F::new(4.0) * t1244 * t1751 * t22243 * t1246 + F::new(6.0) * t1244 * t6238 * t6218 * t1246 - F::new(24.0) * t11888 * t79410 * t11889 - F::new(4.0) * t3624 * t22386 * t22354 + F::new(24.0) * t15027 * t22365 - F::new(12.0) * t15245 * t22372 + F::new(6.0) * t19201 * t6261 + F::new(12.0) * t5064 * t22341 + t79260 * t494 + t79467 + t1244 * t491 * t79008 * t1246 + F::new(6.0) * t3610 * t79453 * t3612 + F::new(12.0) * t3610 * t79461 * t3612 - F::new(36.0) * t44698 * t79473 * t44701 + F::new(24.0) * t44724 * t79473 * t44726 + F::new(4.0) * t22114 * t1758 - F::new(24.0) * t53565 * t22361 + F::new(12.0) * t5064 * t22390 + F::new(6.0) * t6168 * t6265 - F::new(6.0) * t65262 * t6263 + t79524) - F::new(24.0) * t5055 * t22008 - F::new(12.0) * t73891 * t1761;
    t79533
}
