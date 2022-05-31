
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5009(_: S6, _: S4, _: S6) {}
        
        fn test5009() { foo5009(S2, S3, S4, S5, S6, S7, S8); }
    