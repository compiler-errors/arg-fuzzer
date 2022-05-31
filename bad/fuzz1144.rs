
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1144(_: S8, _: S6, _: S3, _: S6) {}
        
        fn test1144() { foo1144(S2, S4, S5, S3, S2); }
    