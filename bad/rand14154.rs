
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14154(_: S3, _: S6, _: S4, _: S6, _: S7, _: S1, _: S0) {}
        
        fn test14154() { foo14154(S1, S2, S3, S4, S5, S6, S7, S8); }
    