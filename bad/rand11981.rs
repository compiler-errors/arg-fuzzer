
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11981(_: S3, _: S5, _: S4) {}
        
        fn test11981() { foo11981(S5, S2, S3, S5, S0); }
    