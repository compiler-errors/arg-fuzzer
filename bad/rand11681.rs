
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11681(_: S4, _: S3, _: S0, _: S5, _: S5, _: S4) {}
        
        fn test11681() { foo11681(S1, S2, S3, S4, S5, S6, S7); }
    