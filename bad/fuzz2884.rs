
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2884(_: S1, _: S5, _: S8) {}
        
        fn test2884() { foo2884(S1, S4, S5, S2); }
    