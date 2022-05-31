
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1230(_: S0, _: S2, _: S5) {}
        
        fn test1230() { foo1230(S0, S1, S2, S7); }
    