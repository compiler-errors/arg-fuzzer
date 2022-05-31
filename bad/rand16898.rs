
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16898(_: S1, _: S2, _: S5, _: S6) {}
        
        fn test16898() { foo16898(S6, S4, S1, S5, S0, S4, S2); }
    