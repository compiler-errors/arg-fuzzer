
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14266(_: S7, _: S4, _: S5) {}
        
        fn test14266() { foo14266(S0, S6, S1, S5, S5, S7, S4); }
    