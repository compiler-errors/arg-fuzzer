
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16232(_: S3, _: S7, _: S2, _: S4, _: S7) {}
        
        fn test16232() { foo16232(S5, S7, S1, S5, S6, S2, S0); }
    