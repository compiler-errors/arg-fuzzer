
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11046(_: S1, _: S2, _: S4, _: S6) {}
        
        fn test11046() { foo11046(S3, S3, S0, S1, S5, S6); }
    