
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11427(_: S1, _: S2, _: S5) {}
        
        fn test11427() { foo11427(S8, S2, S7, S1, S3, S6); }
    