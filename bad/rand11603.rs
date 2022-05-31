
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11603(_: S4, _: S6) {}
        
        fn test11603() { foo11603(S3, S4, S5); }
    