
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo842(_: S1, _: S2, _: S8) {}
        
        fn test842() { foo842(S4, S5, S3, S2, S1, S6); }
    